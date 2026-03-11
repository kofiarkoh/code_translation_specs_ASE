#include <stdio.h>

int main(void){
	int n;
	scanf("%d", &n);
	int k=n;
	int s[15];
	int i=0;
	int sum=0;
	while(k > 0){
		sum += k%10;
		k /= 10;
		i++;
	}
	if(n % sum ==0)printf("Yes\n");
	else printf("No\n");
	return 0;
}
