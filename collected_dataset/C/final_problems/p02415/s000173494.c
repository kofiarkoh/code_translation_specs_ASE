#include <stdio.h>
int main(){
	int i;
	char c[1201];
	gets(c);
	for (i=0;!(c[i]=='.' && c[i+1]!=" ");i++){
		if      (c[i]>='a'&&c[i]<='z') c[i]=c[i]+'A'-'a';
		else if (c[i]>='A'&&c[i]<='Z') c[i]=c[i]-('A'-'a');
	}
	printf("%s\n",c);
	return 0;
}