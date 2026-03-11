#include <stdio.h>
#include <stdbool.h>
#define rep(i,n)for(int i=0;i<(int)(n);i++)
int main(){
	char c[30009]={'0'}; int n, ans=0; bool f1,f2,f3;
	scanf("%d%s",&n,c);
	rep(i, 10) {
		rep(j, 10) {
			rep(k, 10) {
				f1=false;f2=false;f3=false;
				int x=0;
				while(!f1&&x<n){
					f1=(c[x]-48==i);
					x++;
				}
				while(!f2&&x<n){
					f2=(c[x]-48==j);
					x++;
				}
				while(!f3&&x<n){
					f3=(c[x]-48==k);
					x++;
				}
				if(f1&&f2&&f3)ans++;
			}
		}
	}
	printf("%d", ans);
}