#include <stdio.h>
#include <string.h>

int main(void) {
	int p;
	char str[11],ans[11];
	
	scanf("%s",str);
	
	memset(ans,'\0',sizeof(ans));
	p = 0;
	for (int i = 0;str[i] != '\0';i++) {
		if (str[i] == 'B') {
			p--;
			if (p < 0) {
				p = 0;
			}
			ans[p] = '\0';
		}else {
			ans[p++] = str[i];
		}
	}

	printf("%s\n",ans);

	return 0;
}
