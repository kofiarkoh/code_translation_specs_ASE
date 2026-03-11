#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=l;i<r;i++)
#define max(p,q)((p)>(q)?(p):(q))

int main(){
	ll w,h,n,q;
	while(scanf("%lld%lld%lld%lld",&w,&h,&n,&q),n){
		int tate[40],yoko[40];
		rep(i,0,h)tate[i]=1;
		rep(i,0,w)yoko[i]=1;
		while(n--){
			ll c,f;
			scanf("%lld%lld",&f,&c);
			if(f==1){
				//yoko
				int temp[40]={};
				rep(i,0,max(c,w-c)){
					if(c+i<w)temp[i]+=yoko[c+i];
					if(i<c)temp[i]+=yoko[c-1-i];
				}
				w=max(c,w-c);
				rep(i,0,w)yoko[i]=temp[i];
			}else{
				//tate
				int temp[40]={};
				rep(i,0,max(c,h-c)){
					if(c+i<h)temp[i]+=tate[c+i];
					if(i<c)temp[i]+=tate[c-1-i];
				}
				h=max(c,h-c);
				rep(i,0,h)tate[i]=temp[i];
			}
		}
		while(q--){
			ll x,y;
			scanf("%lld%lld",&x,&y);
			printf("%lld\n",yoko[x]*tate[y]);
		}
	}
}
