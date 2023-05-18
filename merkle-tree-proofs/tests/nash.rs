use pba_assignment::nash::*;

/// Player need to coordinate their choice
#[test]
fn coordination() {
    let result = compute_equilibria([1, 0, 0, 2], [1, 0, 0, 2]);
    assert_eq!(result.len(), 2);

    assert!(result.contains(&[1u8, 1u8]));
    assert!(result.contains(&[2u8, 2u8]));
}

/// Classic prisoner dilemma
#[test]
fn prisoner() {
    let result = compute_equilibria([-2, -10, 0, -5], [-2, 0, -10, -5]);

    assert_eq!(result.len(), 1);

    assert!(result.contains(&[2u8, 2u8]));
}

/// Players benefit from doing the opposite of one another
#[test]
fn game_of_chicken() {
    let result = compute_equilibria([-10, 1, -1, 0], [-10, -1, 1, 0]);

    assert_eq!(result.len(), 2);

    assert!(result.contains(&[1u8, 2u8]));
    assert!(result.contains(&[2u8, 1u8]));
}

/// Goal keeper want to jump to the where player shoots
/// Player want to shoot to the opposite site of goal keeper
#[test]
fn penalty_kick() {
    let result = compute_equilibria([0, 1, 1, 0], [1, 0, 0, 1]);

    assert_eq!(result.len(), 0);
}
