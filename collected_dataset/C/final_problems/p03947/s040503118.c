#include <stdio.h>
#include <string.h>


int main(void) {
	char s[999999];
	
	scanf("%s", s);
	
	int l=strlen(s),wcnt=0, i;
	

	
	for(i=1;s[i];i++){
		if(s[i-1]-s[i]){
			wcnt++;
		}
	}
	
	printf("%d\n", wcnt);
	return 0;
}
