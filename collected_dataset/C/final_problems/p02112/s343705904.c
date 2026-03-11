#include <stdio.h>
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

int s[730];
int main(){
	int n;
	scanf("%d",&n);
	while(n--){
		int a,b,c,d;
		scanf("%d%d%d%d",&a,&b,&c,&d);
		a=(a-1)*30+(b-1);
		//[a,a+c)
		for(int i=0;i<360;i++){
			int x=(a+i)%360;
			if(i<c)s[x]=max(s[x],d);
			else s[x]=max(s[x],max(d-(360-i),d-(i-(c-1))));
		}
	}
	int ans=1000;
	for(int i=0;i<360;i++)ans=min(ans,s[i]);
	printf("%d\n",ans);
}
