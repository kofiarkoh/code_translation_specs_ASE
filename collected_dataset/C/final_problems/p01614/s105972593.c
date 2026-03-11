#include <stdio.h>

#define MAX_PHRASES 393
#define MAX_MELODIES 393
#define MAX_LENGTH 394

typedef struct {
	int minimum;
	int maximum;
	int point;
} phrase_t;

typedef struct {
	int score;
} melody_t;

int phrase_num;
phrase_t phrases[MAX_PHRASES];

int score_mem[MAX_LENGTH][MAX_LENGTH];

int get_max_score(int length, int margin, int score)
{
	int i, max_score;

	if(score_mem[length][margin] > 0) {
		return score_mem[length][margin];
	}

	if(length <= margin) {
		max_score = score;
	} else {
		max_score = 0;
	}

	for(i = 0; i < phrase_num; i++) {
		if(length < phrases[i].minimum) break;

		int partial_score = get_max_score(length - phrases[i].minimum, margin + (phrases[i].maximum - phrases[i].minimum), score + phrases[i].point);

		if(partial_score > max_score) {
			max_score = partial_score;
		}
	}

	if(max_score > 0) {
		score_mem[length][margin] = max_score;
	}

	return max_score;
}

int main(void)
{
	{
		int i, j, melody_num;
		melody_t melodies[MAX_MELODIES];

		for(i = 0; i < MAX_LENGTH; i++) {
			for(j = 0; j < MAX_LENGTH; j++) {
				score_mem[i][j] = 0;
			}
		}

		scanf("%d", &phrase_num);

		for(i = 0; i < phrase_num; i++) {
			scanf("%d%d%d", &(phrases[i].minimum), &(phrases[i].maximum), &(phrases[i].point));
		}

		int flag = 0;

		do {
			flag = 0;
			for(i = 0; i < phrase_num - 1; i++) {
				if(phrases[i].minimum > phrases[i+1].minimum) {
					phrase_t temp = phrases[i];
					phrases[i] = phrases[i+1];
					phrases[i+1] = temp;
					flag = 1;
				}
			}
		} while(flag);

		scanf("%d", &melody_num);

		for(i = 0; i < melody_num; i++) {
			int melody_length;

			scanf("%d", &melody_length);

			if((melodies[i].score = get_max_score(melody_length, 0, 0)) == 0) {
				printf("-1\n");
				return 0;
			}
		}

		for(i = 0; i < melody_num; i++) {
			printf("%d\n", melodies[i].score);
		}
	}

	return 0;
}