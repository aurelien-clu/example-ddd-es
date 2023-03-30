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
        let expected = Event::Created {
            registration_id: id.to_string(),
        };
        let services = ();
        PlaneTestFramework::with(services)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![expected]);
    }
}
