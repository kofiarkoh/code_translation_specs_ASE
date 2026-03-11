#include <stdio.h>
#include <stdlib.h>

int a[99],sum[99];
char s[100010];
int main(){
	scanf("%s",s);
	for(int i=0;s[i];i++)a[s[i]-'a']++;
	while(1){
		int m1=-1,m2=-1;
		for(int i=0;i<26;i++)if(a[i]){
			if(m1==-1||a[m1]>a[i])m2=m1,m1=i;
			else if(m2==-1||a[m2]>a[i])m2=i;
		}
		if(m2==-1){
			printf("%d\n",sum[m1]?:a[m1]);
			return 0;
		}
		a[m1]+=a[m2];
		sum[m1]+=sum[m2]+a[m1];
		a[m2]=0;
	}
}
