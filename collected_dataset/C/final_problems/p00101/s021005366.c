#include<stdio.h>
int main(){
  int n,i,j;
  char s[1003];
  scanf("%d",&n);
  if(n==0)return 0;
  for(i=0;i<=n;i++){
    fgets(s,1002,stdin);
    for(j=0;s[j]!=0;){
        if(s[j++]=='H'){
	  /*if(j==1||((!('a'<=s[j-2]&&s[j-2]<='z'))
		&&(!('A'<=s[j-2]&&s[j-2]<='Z'))
		&&(!('0'<=s[j-2]&&s[j-2]<='9')))){*/
	  if(s[j]=='o'){j++;
	    if(s[j]=='s'){j++;
	      if(s[j]=='h'){j++;
		if(s[j]=='i'){j++;
		  if(s[j]=='n'){j++;
		    if(s[j]=='o'){j++;
	  /*if(!('a'<=s[j]&&s[j]<='z')){
        if(!('A'<=s[j]&&s[j]<='Z')){
	if(!('0'<=s[j]&&s[j]<='9')){*/
	  s[j-1]='a';
	}
	}}}}}}//}}}}
    }
    if(i!=0)printf("%s",s);
  }
  return 0;
}