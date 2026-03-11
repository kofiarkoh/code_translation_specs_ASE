#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

typedef struct pair{ll a,b;}P;

P a[100];
ll s[100];
int main(){
	int n,ww,hh;
	while(1){
		scanf("%d%d%d",&n,&ww,&hh);
		if(ww==0)break;
		a[1].a=ww;
		a[1].b=hh;
		rep(i,1,n+1){
			int ii,l;
			scanf("%d%d",&ii,&l);
			int w=a[ii].a,h=a[ii].b;
			l%=h+w;
			rep(j,ii,i)a[j]=a[j+1];
			if(l<w){
				//wを切る
				l=min(l,w-l);
				a[i].a=l;
				a[i].b=h;
				a[i+1].a=w-l;
				a[i+1].b=h;
			}else{
				//hを切る
				l-=w;
				l=min(l,h-l);
				a[i].a=w;
				a[i].b=l;
				a[i+1].a=w;
				a[i+1].b=h-l;
			}
		}
		rep(i,0,n+1)s[i]=a[i+1].a*a[i+1].b;
		sortup(s,n+1);
		rep(i,0,n+1)printf("%lld%c",s[i],i==n?'\n':' ');
	}
	return 0;
}
