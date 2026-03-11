#include <stdio.h>
#include <string.h>

char s[100004];

int main(void){
	int i,n,len,rightFlg,foot[2];
	char before;
	scanf("%d",&n);

	while(n--){
		scanf("%s",s);

		len = strlen(s);
		foot[0] = -1;
		foot[1] = 1;
		rightFlg = 1;
		before = 0;

		for(i=0;i<len;i++){
			if(before == s[i]) break;

			switch(s[i]){
			case 'U':
			case 'D':
				foot[rightFlg] = 0;
				break;
			case 'R': break;
				foot[rightFlg] = 1;
			case 'L': break;
				foot[rightFlg] = -1;
			}

			if(foot[0] > foot[1]) break;

			before = s[i];
			rightFlg = !rightFlg;
		}

		printf("%s\n",i==len ? "Yes" : "No");
	}

	return 0;
}