#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int id;
	int score;
} Pert;

int cmp(const void* x,const void* y) {
	Pert* a;
	Pert* b;
	a=(Pert*)x;
	b=(Pert*)y;
	if(a->score < b->score)return 1;
	if(a->score > b->score)return -1;
	if(a->id > b->id)return 1;
	if(a->id < b->id)return -1;
	return 0;
}

int main(void) {
	while(1){
		int n, i, j;
		scanf("%d", &n);
		if(n==0) break;

		Pert cand[40];
		int score[10][3];
		for(i=0; i<n; i++) {
			scanf("%d",&cand[i].id);
			for(j=0; j<9; j++) {
				scanf("%d", &score[j][0]);
				score[j][1] = 0;
				if(score[j][0]!=10) scanf("%d",&score[j][1]);
			}
			scanf("%d", &score[9][0]);
			scanf("%d", &score[9][1]);
			score[9][2] = 0;
			if(score[9][0]==10 || score[9][0]+score[9][1]==10) scanf("%d",&score[9][2]);

			cand[i].score = 0;
			for(j=0;j<9;j++) {
				cand[i].score += score[j][0] + score[j][1];
				if(score[j][0]==10){
					cand[i].score += score[j+1][0];
					if(score[j+1][0]==10 && j<8) cand[i].score+=score[j+2][0];
					else cand[i].score += score[j+1][1];
				}else if(score[j][0]+score[j][1] == 10) {
					cand[i].score+=score[j+1][0];
				}
			}
			cand[i].score+=score[9][0]+score[9][1]+score[9][2];
		}
		qsort(cand,n,sizeof(Pert),cmp);
		for(i=0;i<n;i++){
			printf("%d %d\n",cand[i].id,cand[i].score);
		}
	}
	return 0;
}
