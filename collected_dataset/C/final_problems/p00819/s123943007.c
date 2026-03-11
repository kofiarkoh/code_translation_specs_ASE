#include<stdio.h>
#include<string.h>
int main(){
  char s[26];
  char c[10];
  int n,i,j,k,l;
  char w;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%s %s",c,s);
    l=strlen(s);
    for(j=strlen(c)-1;j>-1;j--){
      if(c[j]=='C'){
	w=s[0];
	for(k=1;k<l;k++)s[k-1]=s[k];
	s[l-1]=w;
      }
       if(c[j]=='J'){
	w=s[l-1];
	for(k=l-1;k;k--)s[k]=s[k-1];
	s[0]=w;
      }
       if(c[j]=='E'){
	 for(k=0;2*k<l-1;k++){
	   w=s[k];
	   s[k]=s[(l+1)/2+k];
	   s[(l+1)/2+k]=w;
	 }
       }	 
       if(c[j]=='A'){
	 for(k=0;2*k<l-1;k++){
	   w=s[k];
	   s[k]=s[l-k-1];
	   s[l-k-1]=w;
	 }
       }
       if(c[j]=='M'){
	 for(k=0;k<l;k++){
	   if('0'<=s[k]&&s[k]<'9')s[k]++;
	   else if(s[k]=='9')     s[k]='0';
	 }
       }
       if(c[j]=='P'){
	 for(k=0;k<l;k++){
	   if('0'<s[k]&&s[k]<='9')s[k]--;
	   else if(s[k]=='0')     s[k]='9';
	 }
       }
       // printf("%s\n",s);
    }
    printf("%s\n",s);
  }
  return 0;
}