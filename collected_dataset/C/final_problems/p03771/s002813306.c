#include<stdio.h>
#include<stdlib.h>
#include<string.h>


#define N_MAX     200000
#define GROUP_MAX 18
#define DP_SIZE   ((N_MAX+31)/32)



typedef struct tag_group_t
{
	int l, r, is_invalid;
}group_t;


typedef struct
{
	int depth, is_valid_invalid;
	unsigned long flags[DP_SIZE];
}state_t;



typedef int mapdata_t;

typedef struct
{
	unsigned long key[DP_SIZE];
}mapkey_t;

typedef struct
{
	mapkey_t key;
	mapdata_t data;
}mapm_t;


typedef struct
{
	mapm_t **member;
	int member_num, area_size;
}map_t;


map_t *dp[GROUP_MAX+1];
group_t g_group[GROUP_MAX+1][GROUP_MAX+1];
int group_num;
int n, v;
int x[N_MAX];
mapkey_t g_key[GROUP_MAX+1];


mapm_t *mapm_alloc(mapkey_t *key, mapdata_t *data)
{
	mapm_t *mapm;
	mapm=malloc(sizeof(mapm_t));
	mapm->key=*key;
	mapm->data=*data;
	return mapm;
}


void mapm_free(mapm_t *mapm)
{
	free(mapm);
}


void mapm_key_get(mapm_t *mapm, mapkey_t *key)
{
	*key=mapm->key;
}


void mapm_data_get(mapm_t *mapm, mapdata_t *data)
{
	*data=mapm->data;
}


map_t *map_alloc(int area_size)
{
	map_t *map;

	map=malloc(sizeof(map_t));
	if(map!=NULL)
	{
		map->member_num=0;
		map->area_size=area_size;
		map->member=malloc(sizeof(mapm_t*)*area_size);
		if(map->member!=NULL)
		{
			return map;
		}
		free(map);
	}
	return NULL;
}


void map_clear(map_t *map)
{
	int i;

	if(map==NULL) return;

	for(i=map->member_num-1;i>=0;i--)
	{
		mapm_free(map->member[i]);
	}
	map->member_num=0;
}


void map_free(map_t *map)
{
	if(map)
	{
		map_clear(map);
		free(map->member);
		free(map);
	}
}


int map_search(map_t *map, mapkey_t *key, int *index)
{
	int s, e, c, r;

	s=0;
	e=map->member_num-1;
	*index=0;
	for(;;)
	{
		if(s>e) break;
		c=(s+e)/2;
		r=memcmp(key, &(map->member[c]->key), sizeof(mapkey_t));
		if(r<0)
		{
			e=c-1;
			*index=c;
		} else if(r>0) {
			s=c+1;
			*index=s;
		} else {
			*index=c;
			return 1;
		}
	}
	return 0;
}


void map_realloc(map_t *map, int area_size)
{
	mapm_t **prev;

	if(area_size<map->member_num) area_size=map->member_num;
	prev=map->member;
	map->member=realloc(prev, sizeof(mapm_t*)*area_size);
	if(map->member==NULL)
	{
		map->member=prev;
		return;
	}
	map->area_size=area_size;
}


void map_add(map_t *map, mapkey_t *key, mapdata_t *data)
{
	int index;
	mapm_t *mapm;

	if(map_search(map, key, &index))
	{
		map->member[index]->data=*data;
		return;
	}
	if(map->member_num>=map->area_size)
	{
		map_realloc(map, map->area_size+1000);
	}
	mapm=mapm_alloc(key, data);
	memmove(map->member+index, map->member+index+1, sizeof(mapm_t*)*(map->member_num-index));
	map->member[index]=mapm;
	return;
}


int map_get(map_t *map, mapkey_t *key, mapdata_t *data)
{
	int index;

	if(map_search(map, key, &index)==0)
	{
		return 0;
	}
	*data=map->member[index]->data;
	return 1;
}


void bit_set(void *pv, int s, int e)
{
	unsigned long *p;
	int si, sb, ei, eb, i;


	p=pv;
	si=(s>>5);
	ei=(e>>5);
	sb=s&0x1F;
	eb=e&0x1F;
	if(si==ei)
	{
		p[si]|=((1UL<<(eb-sb+1))-1)<<sb;
		return;
	}

	if(sb)
	{
		p[si]|=(0xFFFFFFFFUL<<(sb));
	}
	for(i=si+1;i<ei;i++) p[i]=0xFFFFFFFFUL;
	if(eb)
	{
		p[ei]|=(0xFFFFFFFFUL>>(eb));
	}
}


int bit_get(void *pv, int b)
{
	unsigned long *p;
	int si, sb;

	p=pv;
	si=(b>>5);
	sb=b&0x1F;
	return (p[si]>>sb)&1;
}


int group_max_calc(int v)
{
	int i;

	if(v<0) return 0;
	for(i=0;(1<<i)<=v;i++)
		;
	return i+1;
}


int group_calc(group_t d[], group_t s[], int s_num, int v)
{
	int i, j;
	int d_num=0, d_num_limit;

	d_num_limit=group_max_calc(v);
//printf("v=%d d_num_limit=%d\n", v, d_num_limit);
	for(i=0;i<s_num;i++)
	{
		if(s[i].is_invalid) continue;

		d[d_num].l=s[i].l;
		d[d_num].is_invalid=0;
		for(j=s[i].l;j<s[i].r;j++)
		{
			if(x[j+1]-x[j]>v)
			{
				d[d_num++].r=j;
				if(d_num>=d_num_limit) return -1;
				d[d_num].l=j+1;
				d[d_num].is_invalid=0;
			}
		}
		d[d_num++].r=j;
	}
	return d_num;
}


int calc(group_t group[], int group_num, int depth)
{
	group_t *group2;
	int group2_num;
	int i, v2, ret=0;

	if(map_get(dp[depth], &g_key[depth], &ret)) return ret;

	group2=g_group[depth];
	v2=(v>>depth);
	group2_num=group_calc(group2, group, group_num, v2);
	if(group2_num<0) return 0;

	if(0)
	{
		printf("group_num=%d depth=%d v=%d", group_num, depth, v);
		for(i=0;i<group_num;i++)
		{
//			if(group[i].is_invalid) continue;
			printf(" %d-%d[%d]", group[i].l, group[i].r, group[i].is_invalid);
		}
		printf("\n");
	}

	if(0)
	{
		printf("group2_num=%d depth=%d v2=%d", group2_num, depth, v2);
		for(i=0;i<group2_num;i++)
		{
//			if(group2[i].is_invalid) continue;
			printf(" %d-%d[%d]", group2[i].l, group2[i].r, group2[i].is_invalid);
		}
		printf("\n");
	}

	if(v2<=0)
	{
		return (group2_num<=1);
	}
	if(group2_num<=1)
	{
		return 1;
	}

	for(i=0;i<group2_num;i++)
	{
		g_key[depth+1]=g_key[depth];
		bit_set(&g_key[depth+1], group2[i].l, group2[i].r);
		group2[i].is_invalid=1;
		if(calc(group2, group2_num, depth+1)) ret=1;
		group2[i].is_invalid=0;
	}

	map_add(dp[depth], &g_key[depth], &ret);

	return ret;
}


int main(void)
{
	int i, j, group_num, result;
	group_t temp, group[GROUP_MAX];

	for(i=0;i<=GROUP_MAX;i++) dp[i]=map_alloc(1000);
if(0)
{
	char xxx[100]={0};
	bit_set(xxx, 2, 8);
	for(i=0;i<50;i++) printf("%d", bit_get(xxx, i));
	printf("\n");
	exit(1);
}

	while(scanf("%d%d", &n, &v)==2)
	{
		for(i=0;i<=GROUP_MAX;i++) map_clear(dp[i]);
//		printf("\nn=%d v=%d\n", n, v);
		for(i=0;i<n;i++)
		{
			scanf("%d", &x[i]);
		}
		temp.l=0;
		temp.r=n-1;
		temp.is_invalid=0;
		group_num=group_calc(group, &temp, 1, v);
		for(i=0;i<group_num;i++)
		{
			g_key[1]=g_key[0];
			bit_set(g_key[1].key, group[i].l, group[i].r);
			group[i].is_invalid=1;
			result=calc(group, group_num, 1);
			group[i].is_invalid=0;
			for(j=group[i].l;j<=group[i].r;j++)
			{
				printf("%s\n", result?"Possible":"Impossible");
			}
		}
	}
	return 0;
}
