#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef char key_type[21];

typedef struct record {
    int value;
} record;

typedef struct node {
    void **pointers;
    key_type *keys;
    struct node *parent;
    bool is_leaf;
    int num_keys;
    struct node *next;
} node;

int count = 0;
int order = 64;
node *queue = NULL;

void find_and_print(node *const root, key_type key);
void find_and_print_range(node *const root, key_type range1, key_type range2);
int find_range(node *const root, key_type key_start, key_type key_end,
               key_type returned_keys[], void *returned_pointers[]);
node *find_leaf(node *const root, key_type key);
record *find(node *root, key_type key, node **leaf_out);
#define cut(a) ((a)/2+(a)%2)


record *make_record(int value);
node *make_node(void);
node *make_leaf(void);
int get_left_index(node *parent, node *left);
node *insert_into_leaf(node *leaf, key_type key, record *pointer);
node *insert_into_leaf_after_splitting(node *root, node *leaf, key_type key,
                                       record *pointer);
node *insert_into_node(node *root, node *parent,
                       int left_index, key_type key, node *right);
node *insert_into_node_after_splitting(node *root, node *parent,
                                       int left_index,
                                       key_type key, node *right);
node *insert_into_parent(node *root, node *left, key_type key, node *right);
node *insert_into_new_root(node *left, key_type key, node *right);
node *start_new_tree(key_type key, record *pointer);
node *insert(node *root, key_type key, int value);

int get_neighbor_index(node *n);
node *adjust_root(node *root);
node *coalesce_nodes(node *root, node *n, node *neighbor,
                     int neighbor_index, key_type k_prime);
node *redistribute_nodes(node *root, node *n, node *neighbor,
                         int neighbor_index,
                         int k_prime_index, key_type k_prime);
node *delete_entry(node *root, node *n, key_type key, void *pointer);
node *delete(node *root, key_type key);


void find_and_print(node *const root, key_type key) {

    record *r = find(root, key, NULL);
    if (r == NULL) {
        putc_unlocked('0', stdout);
        putc_unlocked('\n', stdout);
    } else {
        int b = r->value;
        int i;
        if (b<0) {
            putc_unlocked('-', stdout);
            b=-b;
        }
        for (i=1; i*10<=b; i*=10);
        for (;i>0; i/=10) {
            putc_unlocked('0'+b/i, stdout);
            b %= i;
        }
        putc_unlocked('\n', stdout);
    }
}

void find_and_print_range(node *const root, key_type key_start, key_type key_end)
{
  node *n = find_leaf(root, key_start);
  if (n == NULL) return;
  int i;

  for (i = 0; i < n->num_keys && strcmp(n->keys[i], key_start) < 0; i++);

  while (n != NULL) {
    for (; i < n->num_keys && strcmp(n->keys[i], key_end)<=0; i++) {
      printf("%s %d\n", n->keys[i], ((record*)n->pointers[i])->value);
    }
    n = n->pointers[order - 1];
    i = 0;
  }
}


node *find_leaf(node *const root, key_type key) {
    if (root == NULL)  return root;

    int i = 0;
    node *c = root;
    while (!c->is_leaf) {
        i = 0;
        while (i < c->num_keys) {
            if (strcmp(key, c->keys[i])>=0) i++;
            else break;
        }
        c = (node *)c->pointers[i];
    }
    return c;
}


record *find(node *root, key_type key, node **leaf_out) {
    if (root == NULL) {
        if (leaf_out != NULL) *leaf_out = NULL;
        return NULL;
    }

    int i = 0;
    node *leaf = NULL;

    leaf = find_leaf(root, key);

    for (i = 0; i < leaf->num_keys; i++)
        if (strcmp(leaf->keys[i], key)==0) break;

    if (leaf_out != NULL)    *leaf_out = leaf;

    if (i == leaf->num_keys)  return NULL;
    else                      return (record *)leaf->pointers[i];
}

record *make_record(int value)
{
    record *new_record = (record *)malloc(sizeof(record));
    new_record->value = value;
    return new_record;
}


node *make_node(void)
{
    node *new_node;
    new_node = malloc(sizeof(node));
    new_node->keys = malloc((order - 1) * sizeof(key_type));
    new_node->pointers = malloc(order * sizeof(void *));
    new_node->is_leaf = false;
    new_node->num_keys = 0;
    new_node->parent = NULL;
    new_node->next = NULL;
    return new_node;
}


node *make_leaf(void)
{
    node *leaf = make_node();
    leaf->is_leaf = true;
    return leaf;
}



int get_left_index(node *parent, node *left)
{
    int left_index = 0;
    while (left_index <= parent->num_keys &&
           parent->pointers[left_index] != left)
        left_index++;
    return left_index;
}


node *insert_into_leaf(node *leaf, key_type key, record *pointer)
{
    int i, insertion_point;

    insertion_point = 0;
    while (insertion_point < leaf->num_keys && strcmp(leaf->keys[insertion_point] , key) < 0)
        insertion_point++;

    for (i = leaf->num_keys; i > insertion_point; i--) {
        strcpy(leaf->keys[i], leaf->keys[i - 1]);
        leaf->pointers[i] = leaf->pointers[i - 1];
    }
    strcpy(leaf->keys[insertion_point], key);
    leaf->pointers[insertion_point] = pointer;
    leaf->num_keys++;

    return leaf;
}


node *insert_into_leaf_after_splitting(node *root, node *leaf, key_type key, record *pointer)
{
    node *new_leaf;
    key_type *temp_keys;
    void **temp_pointers;
    int insertion_index, split, i, j;
    key_type new_key;
    new_leaf = make_leaf();

    temp_keys = malloc(order * sizeof(key_type));
    temp_pointers = malloc(order * sizeof(void *));

    insertion_index = 0;
    while (insertion_index < order - 1 && strcmp(leaf->keys[insertion_index], key) < 0)
        insertion_index++;

    for (i = 0, j = 0; i < leaf->num_keys; i++, j++) {
        if (j == insertion_index) j++;
        strcpy(temp_keys[j], leaf->keys[i]);
        temp_pointers[j] = leaf->pointers[i];
    }

    strcpy(temp_keys[insertion_index], key);
    temp_pointers[insertion_index] = pointer;

    leaf->num_keys = 0;

    split = cut(order - 1);

    for (i = 0; i < split; i++) {
        leaf->pointers[i] = temp_pointers[i];
        strcpy(leaf->keys[i], temp_keys[i]);
        leaf->num_keys++;
    }

    for (i = split, j = 0; i < order; i++, j++) {
        new_leaf->pointers[j] = temp_pointers[i];
        strcpy(new_leaf->keys[j], temp_keys[i]);
        new_leaf->num_keys++;
    }

    free(temp_pointers);
    free(temp_keys);

    new_leaf->pointers[order - 1] = leaf->pointers[order - 1];
    leaf->pointers[order - 1] = new_leaf;

    for (i = leaf->num_keys; i < order - 1; i++)
        leaf->pointers[i] = NULL;
    for (i = new_leaf->num_keys; i < order - 1; i++)
        new_leaf->pointers[i] = NULL;

    new_leaf->parent = leaf->parent;
    strcpy(new_key, new_leaf->keys[0]);

    return insert_into_parent(root, leaf, new_key, new_leaf);
}


node *insert_into_node(node *root, node *n, int left_index, key_type key, node *right)
{
    int i;

    for (i = n->num_keys; i > left_index; i--) {
        n->pointers[i + 1] = n->pointers[i];
        strcpy(n->keys[i], n->keys[i - 1]);
    }
    n->pointers[left_index + 1] = right;
    strcpy(n->keys[left_index], key);
    n->num_keys++;
    return root;
}


node *insert_into_node_after_splitting(node *root, node *old_node, int left_index, key_type key, node *right)
{
    int i, j, split;
    key_type k_prime;
    node *new_node, *child;
    key_type *temp_keys;
    node **temp_pointers;

    temp_pointers = malloc((order + 1) * sizeof(node *));
    temp_keys = malloc(order * sizeof(key_type));

    for (i = 0, j = 0; i < old_node->num_keys + 1; i++, j++) {
        if (j == left_index + 1) j++;
        temp_pointers[j] = old_node->pointers[i];
    }

    for (i = 0, j = 0; i < old_node->num_keys; i++, j++) {
        if (j == left_index) j++;
        strcpy(temp_keys[j], old_node->keys[i]);
    }

    temp_pointers[left_index + 1] = right;
    strcpy(temp_keys[left_index], key);


    split = cut(order);
    new_node = make_node();
    old_node->num_keys = 0;
    for (i = 0; i < split - 1; i++) {
        old_node->pointers[i] = temp_pointers[i];
        strcpy(old_node->keys[i], temp_keys[i]);
        old_node->num_keys++;
    }
    old_node->pointers[i] = temp_pointers[i];
    strcpy(k_prime, temp_keys[split - 1]);
    for (++i, j = 0; i < order; i++, j++) {
        new_node->pointers[j] = temp_pointers[i];
        strcpy(new_node->keys[j], temp_keys[i]);
        new_node->num_keys++;
    }
    new_node->pointers[j] = temp_pointers[i];
    free(temp_pointers);
    free(temp_keys);
    new_node->parent = old_node->parent;
    for (i = 0; i <= new_node->num_keys; i++) {
        child = new_node->pointers[i];
        child->parent = new_node;
    }

    return insert_into_parent(root, old_node, k_prime, new_node);
}

node *insert_into_parent(node *root, node *left, key_type key, node *right)
{
    int left_index;
    node *parent;

    parent = left->parent;
    if (parent == NULL)
        return insert_into_new_root(left, key, right);

    left_index = get_left_index(parent, left);

    if (parent->num_keys < order - 1)
        return insert_into_node(root, parent, left_index, key, right);

    return insert_into_node_after_splitting(root, parent, left_index, key, right);
}


node *insert_into_new_root(node *left, key_type key, node *right)
{
    node *root = make_node();
    strcpy(root->keys[0], key);
    root->pointers[0] = left;
    root->pointers[1] = right;
    root->num_keys++;
    root->parent = NULL;
    left->parent = root;
    right->parent = root;
    return root;
}


node *start_new_tree(key_type key, record *pointer)
{
    node *root = make_leaf();
    strcpy(root->keys[0], key);
    root->pointers[0] = pointer;
    root->pointers[order - 1] = NULL;
    root->parent = NULL;
    root->num_keys++;
    return root;
}


node *insert(node *root, key_type key, int value)
{
    record *record_pointer = NULL;
    node *leaf = NULL;

    record_pointer = find(root, key, NULL);
    if (record_pointer != NULL) {
        record_pointer->value = value;
        return root;
    }

    record_pointer = make_record(value);

    if (root == NULL)
        return start_new_tree(key, record_pointer);

    leaf = find_leaf(root, key);

    if (leaf->num_keys < order - 1) {
        leaf = insert_into_leaf(leaf, key, record_pointer);
        return root;
    }

    return insert_into_leaf_after_splitting(root, leaf, key, record_pointer);
}


int get_neighbor_index(node *n)
{
    int i;

    for (i = 0; i <= n->parent->num_keys; i++)
        if (n->parent->pointers[i] == n)
            return i - 1;

    return 1;
}


node *remove_entry_from_node(node *n, key_type key, node *pointer) {

    int i, num_pointers;

    i = 0;
    while (strcmp(n->keys[i],  key))
        i++;
    for (++i; i < n->num_keys; i++)
        strcpy(n->keys[i - 1], n->keys[i]);


    num_pointers = n->is_leaf ? n->num_keys : n->num_keys + 1;
    i = 0;
    while (n->pointers[i] != pointer)
        i++;
    for (++i; i < num_pointers; i++)
        n->pointers[i - 1] = n->pointers[i];

    n->num_keys--;

    if (n->is_leaf)
        for (i = n->num_keys; i < order - 1; i++)
            n->pointers[i] = NULL;
    else
        for (i = n->num_keys + 1; i < order; i++)
            n->pointers[i] = NULL;

    return n;
}


node *adjust_root(node *root) {

    node *new_root;

    if (root->num_keys > 0)   return root;

    if (!root->is_leaf) {
        new_root = root->pointers[0];
        new_root->parent = NULL;
    } else
        new_root = NULL;

    free(root->keys);
    free(root->pointers);
    free(root);

    return new_root;
}


node *coalesce_nodes(node *root, node *n, node *neighbor, int neighbor_index, key_type k_prime) {

    int i, j, neighbor_insertion_index, n_end;
    node *tmp;

    if (neighbor_index == -1) {
        tmp = n;
        n = neighbor;
        neighbor = tmp;
    }

    neighbor_insertion_index = neighbor->num_keys;

    if (!n->is_leaf) {

        strcpy(neighbor->keys[neighbor_insertion_index], k_prime);
        neighbor->num_keys++;

        n_end = n->num_keys;

        for (i = neighbor_insertion_index + 1, j = 0; j < n_end; i++, j++) {
            strcpy(neighbor->keys[i], n->keys[j]);
            neighbor->pointers[i] = n->pointers[j];
            neighbor->num_keys++;
            n->num_keys--;
        }

        neighbor->pointers[i] = n->pointers[j];

        for (i = 0; i < neighbor->num_keys + 1; i++) {
            tmp = (node *)neighbor->pointers[i];
            tmp->parent = neighbor;
        }
    } else {
        for (i = neighbor_insertion_index, j = 0; j < n->num_keys; i++, j++) {
            strcpy(neighbor->keys[i], n->keys[j]);
            neighbor->pointers[i] = n->pointers[j];
            neighbor->num_keys++;
        }
        neighbor->pointers[order - 1] = n->pointers[order - 1];
    }

    root = delete_entry(root, n->parent, k_prime, n);
    free(n->keys);
    free(n->pointers);
    free(n);
    return root;
}


node *redistribute_nodes(node *root, node *n, node *neighbor, int neighbor_index, int k_prime_index, key_type k_prime)
{
    int i;
    node *tmp;

    if (neighbor_index != -1) {
        if (!n->is_leaf)
            n->pointers[n->num_keys + 1] = n->pointers[n->num_keys];
        for (i = n->num_keys; i > 0; i--) {
            strcpy(n->keys[i], n->keys[i - 1]);
            n->pointers[i] = n->pointers[i - 1];
        }
        if (!n->is_leaf) {
            n->pointers[0] = neighbor->pointers[neighbor->num_keys];
            tmp = (node *)n->pointers[0];
            tmp->parent = n;
            neighbor->pointers[neighbor->num_keys] = NULL;
            strcpy(n->keys[0], k_prime);
            strcpy(n->parent->keys[k_prime_index], neighbor->keys[neighbor->num_keys - 1]);
        } else {
            n->pointers[0] = neighbor->pointers[neighbor->num_keys - 1];
            neighbor->pointers[neighbor->num_keys - 1] = NULL;
            strcpy(n->keys[0], neighbor->keys[neighbor->num_keys - 1]);
            strcpy(n->parent->keys[k_prime_index], n->keys[0]);
        }
    } else {
        if (n->is_leaf) {
            strcpy(n->keys[n->num_keys], neighbor->keys[0]);
            n->pointers[n->num_keys] = neighbor->pointers[0];
            strcpy(n->parent->keys[k_prime_index], neighbor->keys[1]);
        } else {
            strcpy(n->keys[n->num_keys], k_prime);
            n->pointers[n->num_keys + 1] = neighbor->pointers[0];
            tmp = (node *)n->pointers[n->num_keys + 1];
            tmp->parent = n;
            strcpy(n->parent->keys[k_prime_index], neighbor->keys[0]);
        }
        for (i = 0; i < neighbor->num_keys - 1; i++) {
            strcpy(neighbor->keys[i], neighbor->keys[i + 1]);
            neighbor->pointers[i] = neighbor->pointers[i + 1];
        }
        if (!n->is_leaf)
            neighbor->pointers[i] = neighbor->pointers[i + 1];
    }

    n->num_keys++;
    neighbor->num_keys--;

    return root;
}


node *delete_entry(node *root, node *n, key_type key, void *pointer)
{
    int min_keys;
    node *neighbor;
    int neighbor_index;
    int k_prime_index;
    key_type k_prime;
    int capacity;

    n = remove_entry_from_node(n, key, pointer);

    if (n == root)  return adjust_root(root);

    min_keys = n->is_leaf ? cut(order - 1) : cut(order) - 1;

    if (n->num_keys >= min_keys) return root;

    neighbor_index = get_neighbor_index(n);
    k_prime_index = neighbor_index == -1 ? 0 : neighbor_index;
    strcpy(k_prime, n->parent->keys[k_prime_index]);
    neighbor = neighbor_index == -1 ? n->parent->pointers[1] :
               n->parent->pointers[neighbor_index];

    capacity = n->is_leaf ? order : order - 1;

    if (neighbor->num_keys + n->num_keys < capacity)
        return coalesce_nodes(root, n, neighbor, neighbor_index, k_prime);
    else
        return redistribute_nodes(root, n, neighbor, neighbor_index, k_prime_index, k_prime);
}

node *delete(node *root, key_type key) {
    node *key_leaf = NULL;
    record *key_record = find(root, key, &key_leaf);

    /* CHANGE */
    if (key_record)    count--;

    if (key_record != NULL && key_leaf != NULL) {
        root = delete_entry(root, key_leaf, key, key_record);
        free(key_record);
    }
    return root;
}


int main() {
    node *root = NULL;
    key_type key, L, R;
    int q, n, x;

    scanf("%d", &n);
    for (int i=0; i<n; i++) {
        scanf("%d ", &q);
        switch (q) {
            case 0: // insert
                scanf("%s %d", key, &x);
                root = insert(root, key, x);
                count++;
                break;
            case 1: // find
                scanf("%s\n", key);
                find_and_print(root, key);
                break;
            case 2: // delete key
                scanf("%s\n", key);
                root = delete(root, key);
                break;
            default: // dump
                fscanf(stdin, "%s %s\n", L, R);
                find_and_print_range(root, L, R);
        }
    }
    return 0;
}

