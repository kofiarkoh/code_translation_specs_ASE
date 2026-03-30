#include <stdio.h>
#include <string.h>

#define MAX_PATH_LENGTH 110
#define TARGET_STATE 'B'

// State transition mapping based on the specification
// Assuming states are represented by characters '0' to '5'
char state_transition(char current_state, char input) {
    // Define the state transitions based on the specification
    // This is a placeholder; actual transitions should be defined as per the requirements
    // For example, let's assume:
    // '0' -> '1', '1' -> '2', '2' -> '3', '3' -> '4', '4' -> '5', '5' -> 'B'
    switch (current_state) {
        case '0': return (input == '1') ? '1' : current_state; // Example transition
        case '1': return (input == '2') ? '2' : current_state; // Example transition
        case '2': return (input == '3') ? '3' : current_state; // Example transition
        case '3': return (input == '4') ? '4' : current_state; // Example transition
        case '4': return (input == '5') ? '5' : current_state; // Example transition
        case '5': return (input == 'B') ? 'B' : current_state; // Example transition
        default: return current_state; // Invalid state, remain in current state
    }
}

int main() {
    char input[MAX_PATH_LENGTH];
    
    // Read input paths until a string starting with '#' is encountered
    while (1) {
        fgets(input, MAX_PATH_LENGTH, stdin);
        
        // Check for termination character
        if (input[0] == '#') {
            break;
        }

        // Initialize the current state
        char current_state = '0'; // Assuming starting state is '0'
        int valid_path = 1; // Flag to track if the path is valid

        // Process each character in the input path
        for (int i = 0; input[i] != '\0' && input[i] != '\n'; i++) {
            char input_char = input[i];

            // Check if the input character is a valid index (0 to 5)
            if (input_char < '0' || input_char > '5') {
                valid_path = 0; // Invalid character encountered
                break; // Stop processing the current path
            }

            // Update the current state based on the transition
            current_state = state_transition(current_state, input_char);
        }

        // Output "Yes" if the final state is the target state, otherwise "No"
        if (valid_path && current_state == TARGET_STATE) {
            printf("Yes\n");
        } else {
            printf("No\n");
        }
    }

    return 0;
}
