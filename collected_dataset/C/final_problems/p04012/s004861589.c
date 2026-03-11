#include<stdio.h>
#include<string.h>
int main(void)
{
	int n,k=0,i,j;
	char w[100];
	char al[26]={'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'};
	scanf("%s", w);
	n=strlen(w);
	for(i=0; i<26; i++)
	{
		for(j=0; j<n; j++)
		{
			if(al[i]==w[j])
			k++;
		}
		if(k%2!=0)
		{
			printf("No\n");
			break;
		}
		if(i==25)
		printf("Yes\n");
	}
	
	return 0;
}