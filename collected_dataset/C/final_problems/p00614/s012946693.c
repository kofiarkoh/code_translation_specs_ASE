#include<stdio.h>
int d[666001];
int n[6];
int min(int a,int b){return a<b?a:b;}
int main(){
	int p;
	int i,j;
	int m;
	while(1){
	scanf("%d %d %d %d %d %d %d",&p,&n[0],&n[1],&n[2],&n[3],&n[4],&n[5]);
	if(p==0)return 0;
	m=n[0]+5*n[1]+10*n[2]+50*n[3]+100*n[4]+500*n[5];
	int res;
	int c;
	for(i=0;i<=m;i++){
		res=0;
		c=i;
		res+=min(n[5],(int)c/500);c-=min(n[5],(int)c/500)*500;
		res+=min(n[4],(int)c/100);c-=min(n[4],(int)c/100)*100;
		res+=min(n[3],(int)c/50);c-=min(n[3],(int)c/50)*50;
		res+=min(n[2],(int)c/10);c-=min(n[2],(int)c/10)*10;
		res+=min(n[1],(int)c/5);c-=min(n[1],(int)c/5)*5;
		res+=min(n[0],(int)c);c-=min(n[0],(int)c);
		if(c!=0)res=10000;
		d[i]=res;
	}
	int ans=10000;
	for(i=p;i<=m;i++)ans=min(ans,d[i]+d[i-p]);
	printf("%d\n",ans);
	}
}