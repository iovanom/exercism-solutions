package tree

import (
	"fmt"
	"sort"
)

// Record - The record object
type Record struct {
	ID     int
	Parent int
}

// Node - The node object
type Node struct {
	ID       int
	Children []*Node
}

// Build - build the tree with nodes
func Build(records []Record) (*Node, error) {

	// sort objects
	sort.Slice(records, func(i, j int) bool {
		return records[i].ID < records[j].ID
	})

	// create the record map
	nodeMap := make(map[int]*Node)

	for i, record := range records {

		// check validity
		if record.Parent != 0 && record.ID <= record.Parent || record.ID != i {
			return nil, fmt.Errorf("Record is not valid")
		}

		// add to map
		nodeMap[i] = &Node{ID: record.ID}

		// append child to parent if is not root node
		if record.ID != 0 {
			nodeMap[record.Parent].Children = append(nodeMap[record.Parent].Children,
				nodeMap[record.ID])
		}

	}

	return nodeMap[0], nil
}
