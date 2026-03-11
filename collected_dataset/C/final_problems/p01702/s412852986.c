#include <stdio.h>

int main(void) {
	int N,M,Q;
	while(scanf("%d%d%d",&N,&M,&Q)==3 && (N|M|Q)!=0) {
		char sw[64];
		char op[64];
		char stat[1024];
		char cannot_taiou[1000][36]={{0}};
		int i,j,k;
		for(i=0;i<N;i++)sw[i]=0;
		for(i=0;i<Q;i++) {
			if(scanf("%s%s",op,stat)!=2)return 1;
			for(j=0;j<N;j++) {
				if(op[j]=='1')sw[j]=1-sw[j];
			}
			for(j=0;j<M;j++) {
				for(k=0;k<N;k++) {
					if(stat[j]-'0'!=sw[k])cannot_taiou[j][k]=1;
				}
			}
		}
		for(i=0;i<M;i++) {
			int candidate=-1;
			int candidate_count=0;
			for(j=0;j<N;j++) {
				if(!cannot_taiou[i][j]) {
					candidate=j;
					candidate_count++;
				}
			}
			if(candidate_count!=1)putchar('?');
			else if(candidate<10)putchar(candidate+'0');
			else if(candidate<36)putchar(candidate-10+'A');
			else putchar('!');
		}
		putchar('\n');
	}
	return 0;
}