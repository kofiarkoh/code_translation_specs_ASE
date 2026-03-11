#include <stdio.h>
#include <string.h>


int main(void){
	char str[200000];
	scanf("%s",str);
	int length=strlen(str);

	long long count=1;
	for(int i=0;i<length;i++){
		for(int j=i+1;j<length;j++){
			if(str[i]!=str[j]) count++;
		}
	}
	printf("%lld",count);
	return 0;
} 