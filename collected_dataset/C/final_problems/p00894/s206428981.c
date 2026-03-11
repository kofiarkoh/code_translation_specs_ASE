#include <stdio.h>
#include <limits.h>
#define init(A,B) { int i; for(i=0; i<B; i++) A[i]=0; }

#define P_MAX 1000
#define N_MAX 1000

struct data{ int time; char type; int ID; };

int main(void){
	int N;
	
	while(scanf("%d\n",&N), N!=0){
		int i, j=0, k, t[4], max=0, presence[P_MAX], time[P_MAX]; struct data dat[N_MAX];
		init(presence,P_MAX); init(time,P_MAX);
		
		for(i=0; i<N; i++){
			scanf("%2d/%2d %2d:%2d %c %3d\n",&t[0],&t[1],&t[2],&t[3],&dat[i].type,&dat[i].ID);
			dat[i].time=t[3]+60*(t[2]+24*(t[1]+31*t[0]));
		}
		
		for(i=dat[0].time; i<=dat[N-1].time; i++){
			if(dat[j].time==i && j<N){
				while(dat[j].time==i){
					if(dat[j].type=='I') presence[dat[j].ID]=1;
					else presence[dat[j].ID]=0;
					j++;
				}
			}
			if(presence[0]==1) for(k=1; k<P_MAX; k++) time[k]+=presence[k];
		}
		
		for(i=1; i<P_MAX; i++) if(max<time[i]) max=time[i];
		
		printf("%d\n",max);
	}
	
	return 0;
}