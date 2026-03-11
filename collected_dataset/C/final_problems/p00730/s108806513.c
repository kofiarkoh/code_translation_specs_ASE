#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=l;i<r;i++)
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

ll tate[110],yoko[110];
ll a[110];
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

int main(){
	ll n,w,h;
	while(scanf("%lld%lld%lld",&n,&w,&h),h){
		tate[0]=w;
		yoko[0]=h;
		rep(i,0,n){
			ll p,s;
			scanf("%lld%lld",&p,&s);
			p--;
			s%=tate[p]+yoko[p];
			if(s<tate[p]){
				//tate
				tate[i+1]=min(s,tate[p]-s);
				tate[i+2]=max(s,tate[p]-s);
				yoko[i+1]=yoko[p];
				yoko[i+2]=yoko[p];
			}else{
				//yoko
				s-=tate[p];
				tate[i+1]=tate[p];
				tate[i+2]=tate[p];
				yoko[i+1]=min(s,yoko[p]-s);
				yoko[i+2]=max(s,yoko[p]-s);
			}
			rep(j,p,i+2){
				tate[j]=tate[j+1];
				yoko[j]=yoko[j+1];
			}
		}
		rep(i,0,n+1)a[i]=tate[i]*yoko[i];
		sortup(a,n+1);
		rep(i,0,n+1)printf("%lld%c",a[i],i==n?'\n':' ');
	}
}
