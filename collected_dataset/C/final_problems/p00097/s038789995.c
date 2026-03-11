#include <stdio.h>
int xxx[10][900][101];
int pattern(int n,int s,int from){
    if(n>1){
        int i,x=0;
		if(xxx[n][s][from]>=0)x=xxx[n][s][from];
        else{
		for(i=from; i<100 && i<=s ;i++){x+=pattern(n-1,s-i,i+1);}
		xxx[n][s][from]=x;
		}
        return x;
    }
    else{if(from<=s && s<=100)return 1; else return 0;}
}
 
int main(void){
    int n,s,i,j,k;
	for(i=0;i<10;i++){for(j=0;j<900;j++){for(k=0;k<101;k++){xxx[i][j][k]=-1;}}}
    while(scanf("%d %d",&n,&s)*(n+s)){printf("%d\n",s<900?pattern(n,s,0):0);}
    return 0;
}