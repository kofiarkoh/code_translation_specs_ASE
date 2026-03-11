#include <stdio.h>

#define MAX_PHRASES 393
#define MAX_MELODIES 393

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

int get_max_score(int length, int margin, int score)
{
	int i, max_score;

	if(length <= margin) {
		max_score = score;
	} else {
		max_score = 0;
	}

	for(i = 0; i < phrase_num; i++) {
		if(length < phrases[i].minimum) continue;

		int partial_score = get_max_score(length - phrases[i].minimum, margin + (phrases[i].maximum - phrases[i].minimum), score + phrases[i].point);

		if(partial_score > max_score) {
			max_score = partial_score;
		}
	}

	return max_score;
}

int main(void)
{
	{
		int i, melody_num;
		melody_t melodies[MAX_MELODIES];

		scanf("%d", &phrase_num);

		for(i = 0; i < phrase_num; i++) {
			scanf("%d%d%d", &(phrases[i].minimum), &(phrases[i].maximum), &(phrases[i].point));
		}

		scanf("%d", &melody_num);

		for(i = 0; i < melody_num; i++) {
			int melody_length;

			scanf("%d", &melody_length);

			if((melodies[i].score = get_max_score(melody_length, 0, 0)) == 0) {
				printf("-1");
				return 0;
			}
		}

		for(i = 0; i < melody_num; i++) {
			printf("%d\n", melodies[i].score);
		}
	}

	return 0;
}