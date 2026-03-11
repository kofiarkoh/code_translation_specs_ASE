#include <stdio.h>

int a[256];
int main(){
	char c;
	while(~(c=getchar()))a[c]++;
	int ans=0;
	for(int i='a';i<='z';i++)ans+=a[i]%2;
	printf("%d\n",ans/2);
}
