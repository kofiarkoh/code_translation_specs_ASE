#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TEAMS 10
#define NAME_LENGTH 50

typedef struct {
    char name[NAME_LENGTH];
    int wins;
    int losses;
    int draws;
    int score; // Calculated score
} Team;

// Function to calculate the score based on wins and draws
void calculate_score(Team *team) {
    team->score = team->wins * 3 + team->draws;
}

// Comparison function for sorting teams by score in descending order
int compare_teams(const void *a, const void *b) {
    Team *teamA = (Team *)a;
    Team *teamB = (Team *)b;
    return teamB->score - teamA->score; // Descending order
}

int main() {
    int n;
    
    while (1) {
        // Read the number of teams
        scanf("%d", &n);
        if (n == 0) {
            break; // Terminate input on 0
        }

        // Check for valid number of teams
        if (n < 1 || n > MAX_TEAMS) {
            // Undefined behavior as per specification; no handling implemented
            continue;
        }

        Team teams[MAX_TEAMS];

        // Read team data
        for (int i = 0; i < n; i++) {
            scanf("%s %d %d %d", teams[i].name, &teams[i].wins, &teams[i].losses, &teams[i].draws);
            calculate_score(&teams[i]); // Calculate score for each team
        }

        // Sort teams by score
        qsort(teams, n, sizeof(Team), compare_teams);

        // Output the teams and their scores
        for (int i = 0; i < n; i++) {
            printf("%s,%d\n", teams[i].name, teams[i].score);
        }
    }

    return 0;
}
