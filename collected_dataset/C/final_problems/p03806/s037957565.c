#include<stdio.h>
#include<string.h>
#define FOR(i,a,b) for(int i=a;i<b;i++)
#define SWAP(a,b) {a+=b;b=a-b;a-=b;}
#define INF 1000000000

int n,ma,mb,abc[40][3];
int suma = 0,sumb = 0,sumc = 0,minc = INF;

void roop(int q){
	
	if(minc < sumc) return;

	if(q == n){
		if(ma*sumb == mb*suma && sumc != 0){
			minc = minc < sumc ? minc:sumc;
		}
		return;
	}

	FOR(i,0,2){
		suma += abc[q][0] * i;
		sumb += abc[q][1] * i;
		sumc += abc[q][2] * i;

		roop(q+1);

		suma -= abc[q][0] * i;
		sumb -= abc[q][1] * i;
		sumc -= abc[q][2] * i;
	}
}

int main(){
	scanf("%d %d %d",&n,&ma,&mb);
	FOR(i,0,n) scanf("%d %d %d",&abc[i][0],&abc[i][1],&abc[i][2]);

	FOR(i,0,n-1){
		if(abc[i][2] > abc[i+1][2]){
			SWAP(abc[i][0],abc[i+1][0]);
			SWAP(abc[i][1],abc[i+1][1]);
			SWAP(abc[i][2],abc[i+1][2]);
		}
	}

	roop(0);

	if(minc == INF) printf("-1\n");
	else printf("%d",minc);
}