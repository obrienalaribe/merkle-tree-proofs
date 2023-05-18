
pub fn compute_equilibria(payoffs_one: [i32; 4], payoffs_two: [i32; 4]) -> Vec<[u8; 2]> {
    // Implement your nash solver here

    let mut state: Vec<[u8; 2]> = Vec::new();

    if payoffs_one[0] >= payoffs_one[2] && payoffs_two[0] >= payoffs_two[1] {
        state.push([1, 1]);
    }

    if payoffs_one[1] >= payoffs_one[3] && payoffs_two[0] <= payoffs_two[1] {
        state.push([1, 2]);
    }

    if payoffs_one[0] <= payoffs_one[2] && payoffs_two[2] >= payoffs_two[3] {
        state.push([2, 1]);
    }

    if payoffs_one[1] <= payoffs_one[3] && payoffs_two[2] <= payoffs_two[3] {
        state.push([2, 2]);
    }

    state
}
