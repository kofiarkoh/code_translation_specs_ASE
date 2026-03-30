#include <stdio.h>
#include <string.h>

#define ALPHABET_SIZE 26

// Function to calculate the frequency of characters in a substring
void calculate_frequency(const char *str, int start, int end, int *freq) {
    memset(freq, 0, ALPHABET_SIZE * sizeof(int)); // Reset frequency array
    for (int i = start; i < end; i++) {
        freq[str[i] - 'a']++; // Increment frequency for the character
    }
}

// Function to check if two frequency arrays are equal
int are_frequencies_equal(int *freq1, int *freq2) {
    for (int i = 0; i < ALPHABET_SIZE; i++) {
        if (freq1[i] != freq2[i]) {
            return 0; // Frequencies are not equal
        }
    }
    return 1; // Frequencies are equal
}

// Main function to find the length of the longest anagram substring
int longest_anagram_substring(const char *s1, const char *s2) {
    int len1 = strlen(s1);
    int len2 = strlen(s2);
    int max_length = 0;

    // Frequency array for substrings of s2
    int freq_s2[ALPHABET_SIZE] = {0};

    // Calculate frequency for all substrings of s2
    for (int len = 1; len <= len2; len++) {
        calculate_frequency(s2, 0, len, freq_s2);

        // Check all substrings of s1 of the same length
        for (int start = 0; start <= len1 - len; start++) {
            int freq_s1[ALPHABET_SIZE] = {0};
            calculate_frequency(s1, start, start + len, freq_s1);

            // Compare frequencies
            if (are_frequencies_equal(freq_s1, freq_s2)) {
                max_length = len > max_length ? len : max_length; // Update max length
            }
        }
    }

    return max_length; // Return the maximum length found
}

int main() {
    char s1[4001], s2[4001];

    // Read input strings
    scanf("%4000s %4000s", s1, s2); // Read two strings, ensuring max length

    // Get the length of the longest anagram substring
    int result = longest_anagram_substring(s1, s2);

    // Output the result
    printf("%d\n", result);

    return 0;
}
