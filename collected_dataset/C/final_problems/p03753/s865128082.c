#include<stdio.h>
int map[7][100000];
long a[100000],n,dx[4]={-1,1,0,0},dy[]={0,0,-1,1};


void fill(long i,long j){
	long k;
	map[i][j]=1;
	for(k=0;k<4;k++){
		if(0<=i+dx[k] && i+dx[k]<7 && 0<=j+dy[k] && j+dy[k]<n){
			if(map[i+dx[k]][j+dy[k]]==0) fill(i+dx[k],j+dy[k]);
		}
	}
}

int main(int argc, char const *argv[]){
	long m,q,i,j,c=0;
	scanf("%ld %ld %ld",&n,&m,&q);
	for(i=0;i<q;i++){
		scanf("%ld",&a[i]);
	}
	for(i=0;i<7*n/m;i++){
		for(j=0;j<q;j++){
			map[(m*i+a[j])%7][(m*i+a[j])/7]=1;
		}
	}
	for(i=0;i<7;i++){
		for(j=0;j<n;j++){
			if(map[i][j]==0){
				c++;
				fill(i,j);
			}
		}
	}
	printf("%ld\n",c);
	return 0;
}