#include <stdio.h>

typedef struct {
	int zikoku;
	char name[52];
} station_t;

int main(void) {
	int N,T;
	station_t stations[10][2];
	int i;
	int count;
	scanf("%d%d",&N,&T);
	for(i=0;i<N;i++) {
		int t1,t2;
		scanf("%d:%d%s",&t1,&t2,stations[i][0].name);
		stations[i][0].zikoku=t1*60+t2;
		scanf("%d:%d%s",&t1,&t2,stations[i][1].name);
		stations[i][1].zikoku=t1*60+t2;
	}
	count=0;
	for(i=1;i<N;i++) {
		if(stations[i][0].zikoku-stations[i-1][1].zikoku>=T)count++;
	}
	printf("%d\n",count);
	for(i=1;i<N;i++) {
		int nowtime=stations[i][0].zikoku-stations[i-1][1].zikoku;
		if(nowtime>=T) {
			printf("%s %d\n",stations[i][0].name,nowtime);
		}
	}
	return 0;
}