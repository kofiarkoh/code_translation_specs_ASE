#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))

char s[9000];
ll d[9000];
ll xx[20000],*x=xx+10000;
ll yy[20000],*y=yy+10000;

int main(){
	scanf("%s",s);
	int len=strlen(s);
	
	int cnt=0;
	rep(i,0,len){
		if(s[i]=='F')d[cnt]++;
		else cnt++;
	}
	cnt++;
	
	x[0]=1;
	repp(i,2,cnt,2)if(d[i]){
		//x
		rep(k,-9000,9000-d[i])x[k]|=x[k+d[i]];
		for(ll k=9000;k>=-9000+d[i];k--)x[k]|=x[k-d[i]];
	}
	
	y[0]=1;
	repp(i,1,cnt,2)if(d[i]){
		//y
		rep(k,-9000,9000-d[i])y[k]|=y[k+d[i]];
		for(ll k=9000;k>=-9000+d[i];k--)y[k]|=y[k-d[i]];
	}
		
	int p,q;
	scanf("%d%d",&p,&q);
	puts(abs(p-d[0])<9000&x[p-d[0]]&y[q]?"Yes":"No");
	return 0;
}