#include <stdio.h>

typedef struct {
	int id;
	int t;
} car_t;

int main(void) {
	int m,n;
	while(scanf("%d%d",&m,&n)==2 && (m|n)!=0) {
		car_t chusyazyo[100][2];
		car_t cars[100];
		int i;
		int sim_time;
		int id_to_park;
		int first_car_went=0;
		for(i=0;i<n;i++) {
			cars[i].id=i+1;
			if(scanf("%d",&cars[i].t)!=1)return 1;
		}
		for(i=0;i<m;i++) {
			chusyazyo[i][0].id=chusyazyo[i][0].t=0;
			chusyazyo[i][1].id=chusyazyo[i][1].t=0;
		}
		id_to_park=0;
		for(sim_time=0;;sim_time++) {
			/* zikan keika */
			for(i=0;i<m;i++) {
				if(chusyazyo[i][0].t>0)chusyazyo[i][0].t--;
				if(chusyazyo[i][1].t>0)chusyazyo[i][1].t--;
			}
			/* syukko */
			for(i=0;i<m;i++) {
				if(chusyazyo[i][1].id>0 && chusyazyo[i][1].t<=0) {
					if(first_car_went)putchar(' ');
					printf("%d",chusyazyo[i][1].id);
					chusyazyo[i][1].id=chusyazyo[i][1].t=0;
					first_car_went=1;
				}
				if(chusyazyo[i][1].id==0 && chusyazyo[i][0].id>0 && chusyazyo[i][0].t<=0) {
					if(first_car_went)putchar(' ');
					printf("%d",chusyazyo[i][0].id);
					chusyazyo[i][0].id=chusyazyo[i][0].t=0;
					first_car_went=1;
				}
			}
			/* chusya */
			while(id_to_park<n && id_to_park<sim_time/10) {
				/* aiteru tokoro wo sagasu */
				for(i=0;i<m;i++) {
					if(chusyazyo[i][0].id==0) {
						chusyazyo[i][0]=cars[id_to_park++];
						break;
					}
				}
				if(i<m)continue;
				/* 1. izyou wo sagasu */
				int hi_score=1000000;
				int hi_index=-1;
				for(i=0;i<m;i++) {
					if(chusyazyo[i][1].id==0 && chusyazyo[i][0].t>=cars[id_to_park].t) {
						int score=chusyazyo[i][0].t-cars[id_to_park].t;
						if(score<hi_score) {
							hi_score=score;
							hi_index=i;
						}
					}
				}
				if(hi_index>=0) {
					chusyazyo[hi_index][1]=cars[id_to_park++];
					continue;
				}
				/* 2. sagasu */
				for(i=0;i<m;i++) {
					if(chusyazyo[i][1].id==0) {
						int score=cars[id_to_park].t-chusyazyo[i][0].t;
						if(score<hi_score) {
							hi_score=score;
							hi_index=i;
						}
					}
				}
				if(hi_index>=0) {
					chusyazyo[hi_index][1]=cars[id_to_park++];
					continue;
				}
				/* chusya dekinai */
				break;
			}
			/* syuuryou hantei */
			if(id_to_park>=n) {
				int nokotteru=0;
				for(i=0;i<m;i++) {
					if(chusyazyo[i][0].id>0 || chusyazyo[i][1].id>0)nokotteru=1;
				}
				if(!nokotteru)break;
			}
#if 0
			printf("time = %d\n",sim_time);
			for(i=0;i<m;i++)printf("[%2d,t=%2d]",chusyazyo[i][1].id,chusyazyo[i][1].t);
			puts("");
			for(i=0;i<m;i++)printf("[%2d,t=%2d]",chusyazyo[i][0].id,chusyazyo[i][0].t);
			puts("");
#endif
		}
		putchar('\n');
	}
	return 0;
}