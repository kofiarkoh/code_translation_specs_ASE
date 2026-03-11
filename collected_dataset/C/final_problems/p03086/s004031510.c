#include <stdio.h>
int main (){
char s[11];
int i,j,max=0,k=0,a=0,b=0,c=0,d=0;
scanf ("%s",s);	
for (i=0;s[i]!=0;i++){
	if (s[i]=='A'|| s[i]=='C'|| s[i]=='G'|| s[i]=='T'){
	if (s[i]=='A') a++;
	if (s[i]=='C') b++;
    if (s[i]=='G') c++;
if (s[i]=='T') d++;
	
	k=a+b+c+d;
		
		}
else{
	if (max<k){
	max=k;
	}
	a=0;
	b=0;
	c=0;
	d=0;	
	k=0;
	}
	}
printf ("%d",max);
return 0;
	}