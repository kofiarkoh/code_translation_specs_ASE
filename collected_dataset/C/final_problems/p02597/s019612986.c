#include<stdio.h>
#define min(a, b) ((a) > (b) ? (b) : (a))
#define max(a, b) ((a) > (b) ? (a) : (b))
int a[200200],b[200200],n;
int main(){
	char s[200200];
	scanf("%d%s", &n, s);
	for(int i=0;i<n;i++){
		a[i+1]=a[i]+(s[i]=='W');
		b[n-1-i]=b[n-i]+(s[n-1-i]=='R');
	}
	int ans=n;
	for(int i=0;i<=n;i++){
		ans=min(ans,max(a[i],b[i]));
	}
  printf("%d",ans);
}