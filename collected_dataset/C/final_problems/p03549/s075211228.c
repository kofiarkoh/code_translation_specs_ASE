#include <stdio.h>

int pow2(int a){
	int i=0,ans=1;
	for(i=0;i<a;i++)
		ans*=2;
	return ans;
}

int main(int argc, char const *argv[]){
	int n,m;
	scanf("%d%d",&n,&m);
	printf("%d",(m*1900+(n-m)*100)*pow2(m));
	return 0;
}