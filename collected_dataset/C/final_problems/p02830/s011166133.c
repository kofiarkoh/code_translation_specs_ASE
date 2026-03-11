#include<stdio.h>
#include<string.h>

int main()
{
	int i, len;
  	char a[100], b[100], ab[100]={0};
	
  	
  	if(scanf("%d", &len)==EOF) return -1;
  	if(scanf("%s %s", a, b)==EOF) return -1;
 
 	for(i=0;i<len;i++){
    	ab[2*i]=a[i];
    	ab[2*i+1]=b[i];
    }
  
  	printf("%s\n", ab);
  
  	return 0; 
}