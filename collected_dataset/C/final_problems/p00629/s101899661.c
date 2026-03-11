#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int id;
	int syozoku;
	int solved;
	int penalty;
} team_t;

int qsort_comp(const void* x,const void* y) {
	const team_t* a=(const team_t*)x;
	const team_t* b=(const team_t*)y;
	if((a->solved)<(b->solved))return 1;
	if((a->solved)>(b->solved))return -1;
	if((a->penalty)>(b->penalty))return 1;
	if((a->penalty)<(b->penalty))return -1;
	if((a->id)>(b->id))return 1;
	if((a->id)<(b->id))return -1;
	return 0;
}


int chosen[1001];
int team_num;
team_t teams[300];

int main(void) {
	int i;
	int chosen_num;
	while(1) {
		scanf("%d",&team_num);
		if(team_num==0)break;
		for(i=0;i<team_num;i++) {
			scanf("%d%d%d%d",
				&teams[i].id,&teams[i].syozoku,
				&teams[i].solved,&teams[i].penalty);
		}
		qsort(teams,team_num,sizeof(team_t),qsort_comp);
		memset(chosen,0,sizeof(chosen));
		for(i=0,chosen_num=0;i<team_num;i++) {
			if(chosen_num<10) {
				if(chosen[teams[i].syozoku]<3) {
					printf("%d\n",teams[i].id);
					chosen_num++;
					chosen[teams[i].syozoku]++;
				}
			} else if(chosen_num<20) {
				if(chosen[teams[i].syozoku]<2) {
					printf("%d\n",teams[i].id);
					chosen_num++;
					chosen[teams[i].syozoku]++;
				}
			}if(chosen_num<26) {
				if(chosen[teams[i].syozoku]==0) {
					printf("%d\n",teams[i].id);
					chosen_num++;
					chosen[teams[i].syozoku]++;
				}
			}
		}
	}
	return 0;
}