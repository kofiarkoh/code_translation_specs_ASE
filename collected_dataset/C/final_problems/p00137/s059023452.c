#include <stdio.h>
#include <string.h>

int main(void)
{
	char cfi[15], c[8], a[15];
	int fi, n;
	int i, j, k,len;
	
	scanf ("%d", &n);
	
	for (i = 0; i < n; i++){
		
		scanf("%d", &fi);
		printf("Case %d:\n", i + 1);
		
		for (j = 0; j < 10; j++){
			
			fi *= fi;
			sprintf(cfi,"%d", fi);
			
			len = strlen(cfi);
			if (len != 8){
				len = 8 - len;
				for (k = 0; k < len; k++){
					c[k] = '0';
				}
				c[len] = '\0';
				
				sprintf(a,"%s%s",c,cfi);
			}
			else {
				strcpy(a,cfi);
			}
			fi = 0;
			for (k = 2; k < 6; k++){
				fi = fi * 10 + a[k] - '0';
			}
			printf("%d\n", fi);
		}
	}
	return (0);
}
				
		