#include <stdio.h>

int a[9];
int b[9];
int main(){
	for(int i=0;i<9;i++)scanf("%d",a+i);
	int n;
	scanf("%d",&n);
	for(int _=0;_<n;_++){
		int t;
		scanf("%d",&t);
		for(int i=0;i<9;i++)if(a[i]==t)b[i]=1;
	}
	if(  b[0]&&b[3]&&b[6]
	   ||b[1]&&b[4]&&b[7]
	   ||b[2]&&b[5]&&b[8]
	   ||b[0]&&b[1]&&b[2]
	   ||b[3]&&b[4]&&b[5]
	   ||b[6]&&b[7]&&b[8]
	   ||b[0]&&b[4]&&b[8]
	   ||b[2]&&b[4]&&b[6])puts("Yes");
	else puts("No");
}