use crate::map::Pos2D;
use std::vec;

use super::*;

#[cfg(test)]
mod test_order {
    use super::*;

    #[test]
    fn sort_intents_correctly() {
        let mut im = IntentManager::new();
        // let action = Ac
        let i1 = Intent::test_intent(
            None,
            Some(0),
            None,
            Some(0),
        );
        let i2 = Intent::test_intent(
            None,
            Some(1),
            None,
            Some(1),
        );
        let i3 = Intent::test_intent(
            None,
            Some(2),
            None,
            Some(2),
        );
        im.submit(i2);
        im.submit(i3);
        im.submit(i1);
        // dbg!(im);
        assert_eq!(
            im.queue[0].priority,
            0
        );
        assert_eq!(
            im.queue[1].priority,
            1
        );
        assert_eq!(
            im.queue.last().unwrap().priority,
            2
        );
    }
    #[test]
    fn new_intents_gets_treated_last() {
        let mut im = IntentManager::new();
        let old = Intent::test_intent(
            None,
            Some(0),
            None,
            Some(0),
        );
        let new = Intent::test_intent(
            None,
            Some(0),
            None,
            Some(1),
        );
        im.submit(old);
        im.submit(new);
        assert_eq!(
            im.queue.last().unwrap().entity.unique_id,
            EntityId(0)
        );
    }
    #[test]
    fn remainder_intents_gets_treated_first() {
        let mut im = IntentManager::new();
        let action = Action::test_action(ActionKind::Move);
        let both_intents_processed_first = Intent::test_intent(
            Some(action.clone()),
            Some(0),
            None,
            Some(23),
        );
        let other = Intent::test_intent(
            Some(action),
            Some(0),
            None,
            Some(55),
        );
        im.submit(both_intents_processed_first);
        im.submit(other);
        im.extract_top_intent().unwrap();
        assert_eq!(
            im.extract_top_intent().unwrap().entity.unique_id,
            EntityId(23)
        )
    }
}

#[cfg(test)]
mod test_extract_top_intent {
    use crate::map::Pos2D;

    use super::*;

    #[test]
    fn test_move_one_step_less() {
        // path: vec![Pos2D::new(0, 2), Pos2D::new(0, 1)],
        let intent: Intent = Intent::test_intent(
            None, None, None, None,
        );
        let (a, b) = intent.extract_minimal_intent();

        if let Action::Move(x) = a.action {
            assert_eq!(
                x.path,
                vec![Pos2D::new(0, 2)]
            );
        } else {
            panic!("bad type")
        };
        if let Action::Move(x) = b.expect("some remainder intent expected").action {
            assert_eq!(
                x.path,
                vec![Pos2D::new(0, 1)]
            )
        } else {
            assert!(false, "bad type")
        }
    }
    #[test]
    fn test_move_remainder_can_be_null() {
        let intent: Intent = Intent::test_intent(
            None, None, None, None,
        );
        let (_, y) = intent.extract_minimal_intent();
        let (_, should_be_none) = y.unwrap().extract_minimal_intent();
        assert!(should_be_none.is_none())
    }
    #[test]
    fn priority_kept_for_remainder() {
        let intent: Intent = Intent::test_intent(
            None,
            Some(5),
            None,
            None,
        );
        if let (a, Some(b)) = intent.extract_minimal_intent() {
            assert_eq!(a.priority, 5);
            assert_eq!(b.priority, 5);
        } else {
            panic!("remainder intent expected to be some")
        }
    }
}
