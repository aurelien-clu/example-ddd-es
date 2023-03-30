#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::domain::aggregate::Plane;
    use crate::domain::commands::Command;
    use crate::domain::errors::Error;
    use crate::domain::events::Event;

    type PlaneTestFramework = TestFramework<Plane>;

    #[test]
    fn test_a_plane_should_be_created() {
        let id = "F-TEST";
        let command = Command::Create {
            registration_id: id.to_string(),
        };
        let expected = vec![
            Event::Created {
                registration_id: id.to_string(),
            },
            Event::OnGround,
        ];
        let services = ();
        PlaneTestFramework::with(services)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(expected);
    }

    #[test]
    fn test_a_plane_should_update_its_position() {
        let past = vec![
            Event::Created {
                registration_id: "F-TEST".to_string(),
            },
            Event::OnGround,
        ];
        let command = Command::UpdatePosition {
            latitude: 1.0,
            longitude: 1.0,
            altitude: 1,
        };
        let expected = vec![Event::PositionedAt {
            latitude: 1.0,
            longitude: 1.0,
            altitude: 1,
        }];
        let services = ();
        PlaneTestFramework::with(services)
            .given(past)
            .when(command)
            .then_expect_events(expected);
    }

    #[test]
    fn test_a_plane_should_take_off() {
        let past = vec![
            Event::Created {
                registration_id: "F-TEST".to_string(),
            },
            Event::OnGround,
        ];
        let command = Command::TakeOff;
        let expected = vec![Event::TookOff];
        let services = ();
        PlaneTestFramework::with(services)
            .given(past)
            .when(command)
            .then_expect_events(expected);
    }

    #[test]
    fn test_a_plane_should_land() {
        let past = vec![
            Event::Created {
                registration_id: "F-TEST".to_string(),
            },
            Event::OnGround,
            Event::TookOff,
        ];
        let command = Command::Land;
        let expected = vec![Event::Landed];
        let services = ();
        PlaneTestFramework::with(services)
            .given(past)
            .when(command)
            .then_expect_events(expected);
    }
    //
    // #[test]
    // fn test_a_plane_should_not_take_off_if_not_on_ground() {
    //     let past = vec![
    //         Event::Created {
    //             registration_id: "F-TEST".to_string(),
    //         },
    //         Event::OnGround,
    //     ];
    //     let command = Command:: {
    //     };
    //     let expected = Event:: {
    //     };
    //     let services = ();
    //     PlaneTestFramework::with(services)
    //         .given(past)
    //         .when(command)
    //         .then_expect_events(vec![expected]);
    // }
    //
    // #[test]
    // fn test_a_plane_should_not_land_if_not_in_the_air() {
    //     let past = vec![
    //         Event::Created {
    //             registration_id: "F-TEST".to_string(),
    //         },
    //         Event::OnGround,
    //     ];
    //     let command = Command:: {
    //     };
    //     let expected = Event:: {
    //     };
    //     let services = ();
    //     PlaneTestFramework::with(services)
    //         .given(past)
    //         .when(command)
    //         .then_expect_events(vec![expected]);
    // }
}
