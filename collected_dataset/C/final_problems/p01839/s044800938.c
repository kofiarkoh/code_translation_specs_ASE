#include <stdio.h>

int main(void) {
	int n,i,a = 0,y = 1;
	char s[8];
	scanf("%d",&n);
	for(i = 1;i <= n;i++){
		scanf("%s",s);
		if(s[0] == 'A'){a++;}
		if(s[0] == 'U' && s[1] == 'n'){
			if(a){
				a--;
			}
			else{y = 0;}
		}
	}
	if(a > 0){y = 0;}
	if(y){printf("YES\n");}
	else{printf("NO\n");}
	return 0;
}