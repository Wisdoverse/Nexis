#[cfg(test)]
mod collaboration_test {
    use chrono::{Duration, Utc};
    use nexis_calendar::{detect_overlap, CalendarEvent, EventAttendee, ResponseStatus, TimeRange};
    use nexis_doc::{CRDTDocument, DocMetadata, DocVersion, Document};
    use nexis_meeting::{MediaTrack, SfuConfig, SfuRoom};
    use nexis_task::{DefaultTaskWorkflow, Task, TaskPriority, TaskSource, TaskStatus, TaskWorkflow};
    use uuid::Uuid;

    fn make_document(title: &str, content: &str) -> Document {
        let now = Utc::now();
        let author_id = Uuid::new_v4();

        Document {
            metadata: DocMetadata {
                id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                title: title.to_string(),
                created_by: author_id,
                created_at: now,
                updated_at: now,
            },
            content: content.to_string(),
            current_version: DocVersion {
                version: 1,
                created_at: now,
                created_by: author_id,
                checksum: None,
            },
            crdt_doc: Some(CRDTDocument::new()),
        }
    }

    fn make_task() -> Task {
        let now = Utc::now();
        Task {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            title: "Follow up with design team".to_string(),
            description: Some("Convert meeting notes into next sprint tasks".to_string()),
            status: TaskStatus::Created,
            assigned_to: None,
            block_reason: None,
            priority: TaskPriority::High,
            source: TaskSource::MeetingActionItem,
            due_at: Some(now + Duration::days(2)),
            created_at: now,
            updated_at: now,
        }
    }

    fn make_event(owner_id: Uuid, title: &str, start_offset_minutes: i64, duration_minutes: i64) -> CalendarEvent {
        let base = Utc::now();
        CalendarEvent {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            owner_id,
            title: title.to_string(),
            start_at: base + Duration::minutes(start_offset_minutes),
            end_at: base + Duration::minutes(start_offset_minutes + duration_minutes),
            attendees: vec![EventAttendee {
                member_id: owner_id,
                display_name: "Owner".to_string(),
                response_status: ResponseStatus::Accepted,
                optional: false,
            }],
            source_type: Some("meeting".to_string()),
            source_ref_id: None,
        }
    }

    #[test]
    fn test_meeting_room_lifecycle() {
        let mut room = SfuRoom::new(SfuConfig {
            max_participants: 8,
            video_codec: "vp9".to_string(),
            audio_codec: "opus".to_string(),
        });

        let publisher = room.join_room();
        let subscriber = room.join_room();

        assert_eq!(room.participants().len(), 2);

        let payload = vec![10_u8, 20_u8, 30_u8];
        room.publish_track(publisher, MediaTrack::Video, payload.clone());
        room.subscribe_track(subscriber, MediaTrack::Video);

        assert_eq!(room.latest_payload(publisher, MediaTrack::Video), Some(payload));
        assert!(room.is_subscribed(subscriber, MediaTrack::Video));

        room.leave_room(publisher);

        assert!(!room.participants().contains(&publisher));
        assert_eq!(room.latest_payload(publisher, MediaTrack::Video), None);
        assert!(room.participants().contains(&subscriber));
    }

    #[test]
    fn test_document_crdt_sync() {
        let mut document = make_document("Sprint notes", "Initial draft");

        assert_eq!(document.content, "Initial draft");

        document.content.push_str(" + action items");
        document.metadata.updated_at = Utc::now();
        document.current_version.version += 1;

        let crdt_update = document
            .crdt_doc
            .as_ref()
            .expect("CRDT document should exist")
            .encode_update();

        let replica = CRDTDocument::new();
        replica
            .apply_update(&crdt_update)
            .expect("CRDT update should apply");

        let current_content = document.content.clone();
        assert_eq!(current_content, "Initial draft + action items");
        assert_eq!(document.current_version.version, 2);
        assert_eq!(
            replica.get_content(),
            document
                .crdt_doc
                .as_ref()
                .expect("CRDT document should exist")
                .get_content()
        );
    }

    #[test]
    fn test_task_workflow() {
        let mut task = make_task();
        let assignee_id = Uuid::new_v4();

        let workflow = DefaultTaskWorkflow;
        let transition = workflow
            .transition(TaskStatus::Created, TaskStatus::Assigned)
            .expect("created -> assigned should be valid");
        assert_eq!(transition.from, TaskStatus::Created);
        assert_eq!(transition.to, TaskStatus::Assigned);

        task.assign_to(assignee_id)
            .expect("task assignment should succeed");
        task.start().expect("task start should succeed");
        task.complete().expect("task completion should succeed");

        assert_eq!(task.assigned_to, Some(assignee_id));
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.block_reason.is_none());
    }

    #[test]
    fn test_calendar_conflict_detection() {
        let owner_id = Uuid::new_v4();
        let event_a = make_event(owner_id, "Design Sync", 0, 60);
        let event_b = make_event(owner_id, "Product Review", 30, 45);

        assert!(event_a.overlaps_with(&event_b));

        let overlap = detect_overlap(
            TimeRange::new(event_a.start_at, event_a.end_at),
            TimeRange::new(event_b.start_at, event_b.end_at),
        )
        .expect("events should overlap");

        assert_eq!(overlap.start, event_b.start_at);
        assert_eq!(overlap.end, event_a.end_at);
    }
}
