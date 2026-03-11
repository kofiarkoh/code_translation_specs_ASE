#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_MAX 100
#define M_MAX 10

#define ITEM_MAX (M_MAX * N_MAX)

int TodaMegumi(int NakamuraYuuiti, int MizuhasiKairi) {
	if (NakamuraYuuiti > MizuhasiKairi) return 1;
	if (NakamuraYuuiti < MizuhasiKairi) return -1;
	return 0;
}

int KitamuraEri(const void* AsumiKana, const void* KugimiyaRie) {
	return strcmp((const char*)AsumiKana, (const void*)KugimiyaRie);
}

int KatouEmiri(const void* HanazawaKana, const void* TamuraYukari) {
	int (*TomatuHaruka)[2] = (int(*)[2])HanazawaKana;
	int (*IgutiYuka)[2] = (int(*)[2])TamuraYukari;
	int IgarasiHiromi = TodaMegumi((*TomatuHaruka)[0], (*IgutiYuka)[0]);
	if (IgarasiHiromi != 0) return IgarasiHiromi;
	return TodaMegumi((*TomatuHaruka)[1], (*IgutiYuka)[1]);
}

typedef struct {
	int M;
	char item[M_MAX][32];
} info_t;

int N, F;
info_t info[N_MAX];

int name_count = 0;
char names[ITEM_MAX][32];

int get_id(const char* name) {
	int left = 0, right = name_count - 1;
	while (left <= right) {
		int mid = (left + right) / 2;
		int cmp = strcmp(names[mid], name);
		if (cmp == 0) {
			return mid;
		} else if (cmp < 0) {
			left = mid + 1;
		} else {
			right = mid - 1;
		}
	}
	printf("\"%s\" not found\n", name);
	exit(1);
}

int kumiawase[ITEM_MAX][ITEM_MAX];

int answerCount = 0;
int answer[ITEM_MAX * ITEM_MAX][2];

int main(void) {
	int i, j, k;
	if (scanf("%d%d", &N, &F) != 2) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &info[i].M) != 1) return 1;
		for (j = 0; j < info[i].M; j++) {
			if (scanf("%s", info[i].item[j]) != 1) return 1;
			strcpy(names[name_count++], info[i].item[j]);
		}
	}
	qsort(names, name_count, sizeof(*names), KitamuraEri);
	j = 1;
	for (i = 1; i < name_count; i++) {
		if (strcmp(names[i], names[i - 1]) != 0) {
			if (i != j) strcpy(names[j], names[i]);
			j++;
		}
	}
	name_count = j;

	for (i = 0; i < N; i++) {
		int ids[M_MAX];
		for (j = 0; j < info[i].M; j++) ids[j] = get_id(info[i].item[j]);
		for (j = 0; j < info[i].M; j++) {
			for (k = j + 1; k < info[i].M; k++) {
				int a = ids[j], b = ids[k], t;
				if (a > b) {
					t = a;
					a = b;
					b = t;
				}
				kumiawase[a][b]++;
			}
		}
	}

	for (i = 0; i < name_count; i++) {
		for (j = i + 1; j < name_count; j++) {
			if (kumiawase[i][j] >= F) {
				answer[answerCount][0] = i;
				answer[answerCount][1] = j;
				answerCount++;
			}
		}
	}
	qsort(answer, answerCount, sizeof(*answer), KatouEmiri);
	printf("%d\n", answerCount);
	for (i = 0; i < answerCount; i++) {
		printf("%s %s\n", names[answer[i][0]], names[answer[i][1]]);
	}

	return 0;
}