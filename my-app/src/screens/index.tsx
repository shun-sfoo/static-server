import React, { useState } from 'react';
import { Avatar, Button, List } from 'antd';
import {
  FileZipTwoTone,
  FileMarkdownTwoTone,
  FolderOpenTwoTone,
  FilePdfTwoTone,
  FileUnknownTwoTone,
  FileTextTwoTone,
} from '@ant-design/icons';

export interface PathInfo {
  name: string;
  path_uri: string;
  ext: string;
  is_file: boolean;
  last_modified: string;
}

interface PathProps {
  list: PathInfo[];
  path: String;
  setList: any;
  setPath: any;
  setPre: any;
}

const apiUrl = process.env.REACT_APP_API_URL;

const get_icon = (is_file: boolean, ext: string) => {
  if (is_file) {
    if (ext.toLowerCase() === 'md') {
      return <FileMarkdownTwoTone />;
    } else if (ext.toLowerCase() === 'pdf') {
      return <FilePdfTwoTone />;
    } else if (ext.toLowerCase() === 'zip') {
      return <FileZipTwoTone />;
    } else if (ext.toLowerCase() === 'txt') {
      return <FileTextTwoTone />;
    } else {
      return <FileUnknownTwoTone />;
    }
  }

  return <FolderOpenTwoTone />;
};

export const PathList = ({
  list,
  setList,
  setPre,
  setPath,
  path,
}: PathProps) => {
  return (
    <List
      bordered={true}
      itemLayout="horizontal"
      dataSource={list}
      renderItem={(item) => {
        return (
          <List.Item>
            <List.Item.Meta
              avatar={<Avatar icon={get_icon(item.is_file, item.ext)} />}
              title={
                <Button
                  type="link"
                  onClick={() => {
                    if (item.is_file) {
                      window
                        .fetch(`${apiUrl}/download_file`, {
                          method: 'POST',
                          headers: {
                            'Content-Type': 'application/json',
                          },
                          body: JSON.stringify(item),
                        })
                        .then(async (response) => {
                          response.blob().then((blob) => {
                            let url = window.URL.createObjectURL(blob);
                            let a = document.createElement('a');
                            a.href = url;
                            a.download = item.name;
                            a.click();
                          });
                        });
                    } else {
                      window
                        .fetch(`${apiUrl}/visit_folder`, {
                          method: 'POST',
                          headers: {
                            'Content-Type': 'application/json',
                          },
                          body: JSON.stringify(item),
                        })
                        .then(async (response) => {
                          const data = await response.json();
                          if (response.ok) {
                            console.log(data);
                            setList(data);
                            setPath(item.path_uri);
                            setPre(path);
                          }
                        });
                    }
                  }}
                >
                  {item.name}
                </Button>
              }
              description={item.last_modified}
            />
          </List.Item>
        );
      }}
    />
  );
};
