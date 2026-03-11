#include<stdio.h>
#include<string.h>
int main() {
	char a[10001];
	gets(a);
	int n;
	n=strlen(a);
	int i;
	int num=0,t=0;
	for(i=0; i<n; i++) {
		if(i==0&&a[i]=='p') {
			num--;
			t++;
		} else if(i==0&&a[i]=='g') {
			t++;
		} else if(t!=0&&a[i]=='p') {
			t--;
		} else if(t!=0&&a[i]=='g') {
			num++;
			t--;
		} else if(a[i]=='p') {
			num--;
			t++;
		} else if(a[i]=='g') {
			t++;
		}
	}
	printf("%d",num);
	return 0;
}