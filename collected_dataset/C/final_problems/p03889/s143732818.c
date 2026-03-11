#include<stdio.h>
int main(){
	char s[100010],c[][5]={"bdpq","dbqp"};
	int i,j,l;
	scanf("%s",s);
	for(l=0;s[l];l++);
	for(i=0;s[i];i++){
		for(j=0;c[0][j]-s[i];j++);
		if(s[l-i-1]-c[1][j])break;
	}//printf("%d\n",s[i]);
	printf("%s\n",s[i]+l%2?"No":"Yes");
	return 0;
}