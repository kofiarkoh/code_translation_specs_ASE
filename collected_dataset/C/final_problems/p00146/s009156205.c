#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int i,j,n,num[20],dist[20],wei[20],next[10000000];

double search(int code,int weg,int now){
	
	if(code>=(int)pow(2.0,n)-1)return 0;

	int div=2,sel,i,wgg;
	double ttt=999999999,tim,addt;
	
	if(next[code]>=0){
		sel=now;wgg=weg;i=code;tim=0.0;

		while(i<(int)pow(2.0,n)-1){
			tim+=abs(dist[sel]-dist[next[i]])*(wgg+70)/2000.0;
			sel=next[i];
			wgg+=wei[i];
			i+=(int)pow(2.0,next[i]);
		}
		return tim;

	}
	
	for(i=0;i<n;i++,div*=2){
		if(code%div<div/2){
			if(now==-1)addt=0;
			else addt=abs(dist[now]-dist[i])*(weg+70)/2000.0;
			tim=addt+search(code+div/2,weg+wei[i],i);
			if(ttt>tim){ttt=tim;sel=i;}
		}
	}
	next[code]=sel;
	return ttt;

}

int main(){
	scanf("%d",&n);

	for(i=(int)pow(2.0,n);i>=0;i--)next[i]=-1;
	
	for(i=0;i<n;i++){
		scanf("%d %d %d",&num[i],&dist[i],&wei[i]);
		wei[i]*=20;
	}

	search(0,0,-1);

	for(i=0,j=0;i<n;i++){
		printf("%d%s",num[next[j]],i<n-1?" ":"\n");
		j+=(int)pow(2.0,next[j]);
	}

	return 0;
}