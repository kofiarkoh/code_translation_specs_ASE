#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEBUG 0

typedef struct {
	int head;
	int tail;
} Class;

typedef struct {
	int sub;
	int num;
} Sub;

int compare(const void *a,const void *b)
{
	return ((Class *)a)->head - ((Class *)b)->head;
}

int compareSub(const void *a,const void *b)
{
	return ((Sub *)a)->sub - ((Sub *)b)->sub;
}

void combine(Class class[],int comNum,int cN)
{
	int i=comNum+1;
	while (1) {
		if (class[comNum].head > -1E6) break;
		comNum--;
	}
	while (1) {
		if (class[i].head > -1E6) break;
		i++;
	}

	class[comNum].tail = class[i].tail;
	class[i].head = -1E6;
	class[i].tail = -1E6;
	return;
}

int main(void)
{
	int n,m,i,j;
	long tmp;
	int *x;
	int classNum;
	Class *class;
	Sub *sub;
	scanf("%d %d",&n,&m);
	x = (int *)malloc(sizeof(int) * m);
	class = (Class *)malloc(sizeof(Class) * m);
	sub = (Sub *)malloc(sizeof(Sub) * (m-1));
	for (i=0;i<m;i++) {
		scanf("%d",&x[i]);
		class[i].head = x[i];
		class[i].tail = x[i];
	}
	classNum = m;

	qsort(class,m,sizeof(Class),compare);
	if (DEBUG) {
		printf("===\n");
		for (i=0;i<m;i++) printf("%d-%d ",class[i].head,class[i].tail);
		printf("\n");
	}
	

	for (i=0;i<(m-1);i++) {
		sub[i].sub = class[i+1].head - class[i].tail;
		sub[i].num = i;
	}
	qsort(sub,m-1,sizeof(Sub),compareSub);
	if (DEBUG) {
		printf("===\n");
		for (i=0;i<m-1;i++) printf("[%d]%d ",sub[i].num,sub[i].sub);
		printf("\n");
	}

	for (i=0;classNum>n;i++) {
		combine(class,sub[i].num,classNum);
		classNum--;
		if (DEBUG) printf("!!! [%d]%d-%d\n",sub[i].num,class[sub[i].num].head,class[sub[i].num].tail);
	}

	tmp = 0;
	for (i=0;i<m;i++) {
		if (DEBUG) printf("[%d]%d-%d \n",i,class[i].head,class[i].tail);
		if (class[i].head <= -1E6) continue;
		
		tmp += class[i].tail - class[i].head;
		
		if (DEBUG) printf("tmp = %ld\n",tmp);
	}

	printf("%ld\n",tmp);
	
	free(x);
	free(class);
	free(sub);
	return 0;
}
