/*id�̃\�[�g���K�v*/


#include<stdio.h>
#include<string.h>
#define N 100000

typedef struct{

	int id;
	int p;
	int d;
	int k;
	char type[20];
	int c[20];
}node;

node no[N];

int main()
{
	int i,j,n,l=0,m,tmp;
  
	while(1)
	{
		scanf("%d",&n);
		if(n >= 1 && n <= N) break;
	}

	for(i = 0;i < n; i++)
	{
		no[i].p = -1;
		no[i].d = 1;
		scanf("%d%d",&no[i].id,&no[i].k);
		for(j = 0; j < no[i].k; j++) scanf("%d",&no[i].c[j]);
	}

	for(i = 0;i < n;i++)
	{
		for(j = i+1;j < n;j++)
		{
			if(no[i].id > no[j].id)
			{
				tmp = no[i].id;
				no[i].id = no[j].id;
				no[j].id = tmp;
				
				tmp = no[i].k;
				no[i].k = no[j].k;
				no[j].k = tmp;


				if(no[i].k - no[j].k >= 0) l = no[i].k;
				else l = no[j].k;

				for(m = 0;m < l;m++)
				{
					tmp = no[i].c[m];
					no[i].c[m] = no[j].c[m];
					no[j].c[m] = tmp;
				}
			}
		}
	}

	for(i=0;i<n;i++)
	{
		if(no[i].k != 0) strcpy(no[i].type,"internal node");
		else strcpy(no[i].type,"leaf");
	}

	for(i = 0; i < n; i++)
	{
		for(j = 0; j < n; j++)
			for(m = 0; m < no[j].k; m++)
				if(no[i].id == no[j].c[m]) no[i].p = no[j].id;
	}


	for(i = 0;i < n;i++)
	{
		j = no[i].p;
		if(j == -1) continue;
		while(no[j].p != -1)
		{
			no[i].d += 1;
			j = no[j].p;
		}
	}




/*
	for(i = 0; i < n; i++)
	{
		if(no[i].p == -1) continue;
		if(strcmp(no[i].type,"internal node") == 0) no[i].d = no[no[i].p].d + 1;
	}
	for(i = 0;i < n;i++)
	{
		if(no[i].p == -1) continue;
		if(strcmp(no[i].type,"leaf") == 0) no[i].d = no[no[i].p].d + 1;
	}
*/
	for(i = 0;i < n;i++)
	{
		if(no[i].p == -1)
		{
			strcpy(no[i].type,"root");
			no[i].d = 0;
		}
	}
	
	for(i = 0; i < n; i++)
	{
		printf("node %d: parent = %d, depth = %d, %s, [",no[i].id,no[i].p,no[i].d,no[i].type);
		if(no[i].k == 0) printf("]\n");
		else
		{
			for(j = 0; j < no[i].k - 1; j++) printf("%d, ",no[i].c[j]);
			printf("%d]\n",no[i].c[j]);
		}
	}

 	 return 0;
}
