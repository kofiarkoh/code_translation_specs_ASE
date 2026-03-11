#include <stdio.h>

int main(void) {
	int n;
	while(scanf("%d",&n)==1 && n!=0) {
		char input[500][12];
		int P[26][26]={{0}}; /* [] > [] */
		int i,j,k;
		int ok=1;
		for(i=0;i<n;i++)scanf("%s",input[i]);
		/* zyunzyho kankei wo chuusyutu suru */
		for(i=0;i<n;i++) {
			for(j=i+1;j<n;j++) {
				int k;
				for(k=0;input[i][k]!='\0' || input[j][k]!='\0';k++) {
					if(input[i][k]!=input[j][k]) {
						if(input[i][k]!='\0') {
							if(input[j][k]!='\0') {
								P[input[j][k]-'a'][input[i][k]-'a']=1;
							} else {
								/* no way, example: ab < a */
								ok=0;
							}
						}
						break;
					}
				}
			}
		}
		/* suiiritu */
		for(k=0;k<26;k++) {
			for(i=0;i<26;i++) {
				for(j=0;j<26;j++) {
					if(P[i][k]!=0 && P[k][j]!=0)P[i][j]=1;
				}
			}
		}
		/* check */
		for(i=0;i<26;i++) {
			if(P[i][i]!=0)ok=0;
		}
		puts(ok?"yes":"no");
	}
	return 0;
}